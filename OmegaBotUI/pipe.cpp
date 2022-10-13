#include "pipe.h"

#include <QMessageBox>

Pipe::Pipe(LPCSTR name)
    : hPipe(INVALID_HANDLE_VALUE), name(name) {}

Pipe::~Pipe()
{
    Disconnect();
}

void Pipe::Create()
{
    //                            ACCESS TYPE         DATA TYPE                                              MAX INSTANCES              SIZE OF OUTBOUND AND INBOUND
    hPipe = CreateNamedPipeA(name, PIPE_ACCESS_DUPLEX, PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT, 1, 1024 * sizeof(wchar_t), 1024 * sizeof(wchar_t), NMPWAIT_USE_DEFAULT_WAIT, NULL);
    if (hPipe == INVALID_HANDLE_VALUE)
        QMessageBox().warning(0, "Error", QString("Failed to create named pipe, error code: %1").arg(GetLastError()));
    else if (ConnectNamedPipe(hPipe, NULL) != FALSE)
        connected = true;
    else
        QMessageBox().warning(0, "Error", QString("Failed to connect named pipe, error code: %1").arg(GetLastError()));
}

bool Pipe::SendMSG(MessageType type, bool wait, QString* error)
{
    if (!connected) return false;

    mutex.lock();
    BOOL fSuccess = FALSE;
    DWORD cbToWrite = sizeof(wchar_t) * 2, cbWritten;
    wchar_t msg[2] = { (wchar_t)type, 0 };

    fSuccess = WriteFile(hPipe, msg, cbToWrite, &cbWritten, NULL);
    mutex.unlock();
    if (!fSuccess && error) *error = QString("%1").arg(GetLastError());

    if (fSuccess && wait) {
        QString response = ReceiveMSG();
        MessageType type = (MessageType)response.at(0).cell();
        if (type != Received) {
            fSuccess = false;
            if (type == Error && error) {
                *error = response.remove(0, 1);
            }
        }
    }

    return fSuccess;
}

bool Pipe::SendMSG(const QString& message, bool wait, QString* error)
{
    if (!connected) return false;

    mutex.lock();
    BOOL fSuccess = FALSE;
    DWORD cbToWrite = (message.length() + 1) * sizeof(wchar_t), cbWritten;
    wchar_t* msg = (wchar_t*)malloc(cbToWrite);
    message.toWCharArray(msg);
    msg[message.length()] = 0;

    fSuccess = WriteFile(hPipe, msg, cbToWrite, &cbWritten, NULL);
    mutex.unlock();
    if (!fSuccess && error) *error = QString("%1").arg(GetLastError());

    if (fSuccess && wait) {
        QString response = ReceiveMSG();
        MessageType type = (MessageType)response.at(0).cell();
        if (type != Received) {
            fSuccess = false;
            if (type == Error && error) {
                *error = response.remove(0, 1);
            }
        }
    }

    return fSuccess;
}

bool Pipe::SendMSG(MessageType type, const QString& message, bool wait, QString* error)
{
    if (!connected) return false;

    mutex.lock();
    BOOL fSuccess = FALSE;
    DWORD cbToWrite = (message.length() + 2) * sizeof(wchar_t), cbWritten;
    wchar_t* msg = (wchar_t*)malloc(cbToWrite);
    msg[0] = type;
    message.toWCharArray(msg + 1);
    msg[message.length() + 1] = 0;

    fSuccess = WriteFile(hPipe, msg, cbToWrite, &cbWritten, NULL);
    mutex.unlock();
    if (!fSuccess && error) *error = QString("%1").arg(GetLastError());

    if (fSuccess && wait) {
        QString response = ReceiveMSG();
        MessageType type = (MessageType)response.at(0).cell();
        if (type != Received) {
            fSuccess = false;
            if (type == Error && error) {
                *error = response.remove(0, 1);
            }
        }
    }

    return fSuccess;
}

QString Pipe::ReceiveMSG()
{
    if (!connected) return "";

    mutex.lock();
    WCHAR buffer[MAX_PATH + 2];
    DWORD dwRead;

    if (!ReadFile(hPipe, buffer, sizeof(buffer) - sizeof(WCHAR), &dwRead, NULL)) return "";
    mutex.unlock();

    buffer[dwRead / sizeof(WCHAR)] = 0;
    return QString::fromWCharArray(buffer, (dwRead / sizeof(WCHAR)) != 0 ? (dwRead / sizeof(WCHAR)) : -1);
}

void Pipe::Disconnect()
{
    DisconnectNamedPipe(hPipe);
    CloseHandle(hPipe);
    hPipe = INVALID_HANDLE_VALUE;
    connected = false;
}

bool Pipe::Exists() const
{
    return hPipe != INVALID_HANDLE_VALUE && connected;
}
