#ifndef PIPE_H
#define PIPE_H

#include <windows.h>
#include <mutex>

#include <QString>

class Pipe
{
public:
    enum MessageType
    {
        Ping=1,
        Error,
        Received,
        Exit,

        ChangeFPS,
        Speedhack,
        RespawnTime,
        FrameAdvance,
        PracticeFix,
        SetNoClip,

        StartPlayback,
        StopPlayback,
        StartRecording,
        StopRecording,
        Append,

        SaveReplay,
        LoadReplay,

        ApplyHack,
        RestoreHack,

        SetReplayType,

        SetStraightFlyAccuracy,
        SetStraightFlyPlayer,
        SetStraightFlyKeybind,
        SetSpamPress,
        SetSpamRelease,
        SetSpamPlayer,
        SetSpamKeybind,
        IgnoreInput,
    };
private:
    HANDLE hPipe;
    LPCSTR name;
    std::mutex mutex;
    bool connected = false;
public:
    Pipe(LPCSTR name);
    ~Pipe();

    void Create();
    bool SendMSG(MessageType type, bool wait=false, QString* error=nullptr);
    bool SendMSG(const QString& message, bool wait=false, QString* error=nullptr);
    bool SendMSG(MessageType type, const QString& message, bool wait=false, QString* error=nullptr);
    QString ReceiveMSG();
    void Disconnect();

    bool Exists() const;
};

#endif // PIPE_H
