QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++17

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    main.cpp \
    mainwindow.cpp \
    memoryhandler.cpp \
    pipe.cpp \
    runguard.cpp \
    titlebar.cpp

HEADERS += \
    mainwindow.h \
    memoryhandler.h \
    pipe.h \
    runguard.h \
    titlebar.h \
    version.h

FORMS += \
    mainwindow.ui

LIBS += \
    -luser32 \
    -lAdvapi32 \
    -lkernel32

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

DISTFILES += \
    logo.ico \
    resources.rc

RC_FILE = resources.rc

RC_ICONS = logo.ico

RESOURCES += \
    resources.qrc

# For replay api bindings
LIBS += \
    -lws2_32 \
    -lBcrypt \
    -luserenv

win32: LIBS += -L$$PWD/replay/ -lreplay

INCLUDEPATH += $$PWD/replay
DEPENDPATH += $$PWD/replay

win32:!win32-g++: PRE_TARGETDEPS += $$PWD/replay/replay.lib
else:win32-g++: PRE_TARGETDEPS += $$PWD/replay/replay.a
