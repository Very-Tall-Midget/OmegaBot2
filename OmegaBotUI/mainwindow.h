#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QQueue>
#include <QWidget>
#include <QRegExpValidator>

#include "memoryhandler.h"
#include "pipe.h"
#include "titlebar.h"

#define CHECK_INJECTED if (!injected) { Error("Not injected!"); return; }

enum class HackName {
    NoSpike = 1,
    PracticeMusicFix,
    AntiCheatBypass,
    NoRespawnFlash,
    IgnoreEscape,
    DisableDeathEffect,
};

class ErrorForwarder : public QObject {
    Q_OBJECT
public:
    void ForwardError(const QString& error) {
        emit Error(error);
    }
signals:
    void Error(const QString& error);
};

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT
public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
private:
    void LoadTheme(const QColor& gray, const QColor& darkGray, const QColor& black, const QColor& blue);

    void Attach();
    void EnableAll(bool enable);
    void Inject();
    void Uninject();

    bool SendMessages(QString* error);
    template<typename T>
    void QueueMessage(T&& msg);
    void ParseMessage(const QString& msg);

    void Error(const QString& errorMessage);
private slots:
    void on_injectButton_clicked();
    void on_recordButton_clicked();
    void on_playButton_clicked();
    void on_replayTypeComboBox_currentIndexChanged(int index);
    void on_frameAdvanceCheckBox_stateChanged(int state);
    void on_setFPSButton_clicked();
    void on_accuracyFixCheckBox_stateChanged(int state);
    void on_practiceFixCheckBox_stateChanged(int state);
    void on_setRespawnTimeButton_clicked();
    void on_setSpeedButton_clicked();
    void on_antiCheatBypassCheckBox_stateChanged(int state);
    void on_practiceMusicFixCheckBox_stateChanged(int state);
    void on_ignoreESCCheckBox_stateChanged(int state);
    void on_noRespawnFlashCheckBox_stateChanged(int state);
    void on_disableDeathEffectCheckBox_stateChanged(int state);
    void on_speedLinkButton_toggled(bool checked);
    void on_noClipSpinBox_currentIndexChanged(int index);
    void on_pressIntervalSpinBox_valueChanged(int frames);
    void on_releaseIntervalSpinBox_valueChanged(int arg1);
    void on_spamPlayerComboBox_currentIndexChanged(int index);
    void on_spamKeybindLineEdit_textChanged(const QString &arg1);
    void on_straightFlyAccuracySpinBox_valueChanged(int arg1);
    void on_straightFlyPlayerComboBox_currentIndexChanged(int index);
    void on_straightFlyKeybindLineEdit_textChanged(const QString &arg1);
private:
    MemoryHandler memoryHandler;
    QQueue<std::function<bool(QString* error)>> messageQueue;
    Pipe pipe;
    bool injected = false, recording = false, playing = false;
    QWidget* errorParent = nullptr;
    ErrorForwarder* errorForwarder;
    TitleBar* titleBar;
    QRegExpValidator* keybindValidator;

    Ui::MainWindow *ui;
};
#endif // MAINWINDOW_H
