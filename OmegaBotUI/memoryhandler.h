#ifndef MEMORY_HANDLER_H
#define MEMORY_HANDLER_H

#include <Windows.h>
#include <iostream>
#include <string>
#include <vector>
#include <Psapi.h>
#include <TlHelp32.h>
#include <cstdio>

#include <QDir>

#define ReCa reinterpret_cast

class MemoryHandler
{
private:
    HWND window = 0;
    DWORD processID = 0;
    HANDLE hProcess = 0;
public:
    MemoryHandler() = default;
    MemoryHandler(PCSTR windowName);

    HANDLE GetProcess() { return hProcess; }

    bool IsInitialised() { return processID != 0 && hProcess != 0; }

    template<typename T>
    T Read(uint32_t vaddress) const { T buf; return ReadProcessMemory(hProcess, ReCa<void*>(vaddress), &buf, sizeof(T), NULL) ? buf : T(); }

    template<typename T>
    bool Write(uint32_t vaddress, const T& value) const { return WriteProcessMemory(hProcess, ReCa<void*>(vaddress), &value, sizeof(T), NULL); }
    bool Write(uint32_t vaddress, const void* bytes, size_t size) const { return WriteProcessMemory(hProcess, ReCa<void*>(vaddress), bytes, size, NULL); }

    uint32_t Protect(uint32_t vaddress, size_t size, DWORD newProtect) const;
    uint32_t Allocate(size_t size, uint32_t vaddress=0) const;

    bool NewThread(uint32_t vaddress, void* param=nullptr) const;
    bool Inject(const wchar_t* dllPath) const;

    void SwitchFocus();

    uint32_t GetPointerAddress(std::vector<uint32_t> offsets, uint32_t moduleBase) const;
    uint32_t GetModuleBase(const wchar_t* module) const;
    DWORD GetProcessID(LPCSTR processName) const;
    HANDLE GetProcessByName(LPCSTR name) const;
    uint32_t GetProcAddressEx(const wchar_t* module, const char* function) const;
};

#endif // MEMORY_HANDLER_H
