#include "mainwindow.h"
#include "runguard.h"

#include <QApplication>

int main(int argc, char *argv[])
{
    qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", "2");

    RunGuard guard("OmegaBotInstance");
    if (!guard.tryToRun())
    {
        HWND existingInstance = FindWindow(0, L"OmegaBot");
        if (existingInstance) SetForegroundWindow(existingInstance);
        return 0;
    }

    QApplication a(argc, argv);

    a.setAttribute(Qt::AA_EnableHighDpiScaling);
    a.setAttribute(Qt::AA_UseHighDpiPixmaps);

    MainWindow w;
    w.show();
    return a.exec();
}
