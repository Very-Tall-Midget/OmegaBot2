#include "memoryhandler.h"

MemoryHandler::MemoryHandler(PCSTR windowName)
    : window(FindWindowA(0, windowName)), processID(GetProcessID(windowName)), hProcess(GetProcessByName(windowName)) {}

uint32_t MemoryHandler::Protect(uint32_t vaddress, size_t size, DWORD newProtect) const
{
    DWORD old;
    return VirtualProtectEx(hProcess, ReCa<void*>(vaddress), size, newProtect, &old) ? static_cast<uint32_t>(old) : 0;
}

uint32_t MemoryHandler::Allocate(size_t size, uint32_t vaddress) const
{
    return ReCa<uint32_t>(VirtualAllocEx(hProcess, ReCa<void*>(vaddress), size, MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE));
}

bool MemoryHandler::NewThread(uint32_t vaddress, void *param) const
{
    return CreateRemoteThread(hProcess, 0, 0, ReCa<LPTHREAD_START_ROUTINE>(vaddress), param, 0, 0);
}

bool MemoryHandler::Inject(const wchar_t* dllPath) const
{
    uint32_t addr = Allocate((wcslen(dllPath) + 1) * sizeof(wchar_t));
    if (addr && Write(addr, dllPath, (wcslen(dllPath) + 1) * sizeof(wchar_t)))
        return NewThread(ReCa<uint32_t>(LoadLibraryW), ReCa<void*>(addr));
    return false;
}

void MemoryHandler::SwitchFocus()
{
    SetFocus(window);
}

uint32_t MemoryHandler::GetPointerAddress(std::vector<uint32_t> offsets, uint32_t moduleBase) const
{
    if (offsets.size() > 1)
    {
        uint32_t buf = Read<uint32_t>(offsets[0] + moduleBase);

        for (size_t i = 1; i < offsets.size() - 1; ++i)
            buf = Read<uint32_t>(buf + offsets[i]);
        return buf + offsets.back();
    }
    return offsets.size() ? offsets[0] + moduleBase : 0;
}

uint32_t MemoryHandler::GetModuleBase(const wchar_t* module) const
{
    static const int size = 0x1000;
    DWORD out;
    HMODULE hmods[size];
    if (EnumProcessModulesEx(hProcess, hmods, 0x1000, &out, LIST_MODULES_ALL))
    {
        for (uint32_t i = 0; i < out / 4; ++i)
        {
            wchar_t path[MAX_PATH];
            if (GetModuleBaseNameW(hProcess, hmods[i], path, MAX_PATH))
            {
                if (wcscmp(path, module) == 0)
                    return ReCa<uint32_t>(hmods[i]);
            }
        }
    }
    return 0;
}

DWORD MemoryHandler::GetProcessID(LPCSTR processName) const
{
    HWND hwnd = FindWindowA(0, processName);
    if (!hwnd)
        return 0;
    DWORD procID;
    GetWindowThreadProcessId(hwnd, &procID);
    return procID;
}

HANDLE MemoryHandler::GetProcessByName(LPCSTR name) const
{
    DWORD procID = GetProcessID(name);
    if (!procID) return 0;
    return OpenProcess(PROCESS_ALL_ACCESS, FALSE, procID);
}

uint32_t MemoryHandler::GetProcAddressEx(const wchar_t* module, const char* function) const
{
    if (!module || !function)
        return 0;

    uint32_t moduleBase = GetModuleBase(module);

    if (!moduleBase)
        return 0;

    IMAGE_DOS_HEADER Image_Dos_Header = Read<IMAGE_DOS_HEADER>(moduleBase);

    if (Image_Dos_Header.e_magic != IMAGE_DOS_SIGNATURE)
        return 0;

    IMAGE_NT_HEADERS Image_Nt_Headers = Read<IMAGE_NT_HEADERS>(moduleBase + Image_Dos_Header.e_lfanew);

    if (Image_Nt_Headers.Signature != IMAGE_NT_SIGNATURE)
        return 0;

    uint32_t img_exp_dir_rva = 0;

    if (!(img_exp_dir_rva = Image_Nt_Headers.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress))
        return 0;

    IMAGE_EXPORT_DIRECTORY Image_Export_Directory = Read<IMAGE_EXPORT_DIRECTORY>(moduleBase + img_exp_dir_rva);

    uint32_t EAT = moduleBase + Image_Export_Directory.AddressOfFunctions;
    uint32_t ENT = moduleBase + Image_Export_Directory.AddressOfNames;
    uint32_t EOT = moduleBase + Image_Export_Directory.AddressOfNameOrdinals;

    WORD ordinal = 0;
    SIZE_T len_buf = strlen(function) + 1;
    char* temp_buf = new char[len_buf];

    for (size_t i = 0; i < Image_Export_Directory.NumberOfNames; i++)
    {
        uint32_t tempRvaString = Read<uint32_t>(ENT + (i * sizeof(uint32_t)));

        if (!ReadProcessMemory(hProcess, ReCa<LPCVOID>(moduleBase + tempRvaString), temp_buf, len_buf, nullptr))
            return 0;

        if (!lstrcmpiA(function, temp_buf))
        {
            ordinal = Read<WORD>(EOT + (i * sizeof(WORD)));

            uint32_t temp_rva_func = 0;

            temp_rva_func = Read<uint32_t>(EAT + (ordinal * sizeof(uint32_t)));

            delete[] temp_buf;
            return moduleBase + temp_rva_func;
        }
    }
    delete[] temp_buf;
    return 0;
}
