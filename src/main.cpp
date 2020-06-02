#include "Bindings.h"

#include <QtCore/QFile>
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>
#include <QtQuick/QQuickItem>
#include <QtQuick/QQuickWindow>
#include <QtQml/qqml.h>

#include <stdio.h>

// exported functions
extern "C" {
    int main_cpp(const char* app); // the function C++ source exports (and Rust source calls it)
    void set_qmlglsink_widget(QQuickItem* widget);// the function Rust source exports (and this source calls it)
}

int main_cpp(const char* appPath)
{
    int argc = 1;
    char* argv[1] = { (char*)appPath };
    QGuiApplication app(argc, argv);
    // qmlRegisterType<Simple>("RustCode", 1, 0, "Simple");

    QQmlApplicationEngine engine;
    if (QFile("main.qml").exists()) {
        engine.load(QUrl(QStringLiteral("main.qml")));
    } else {
        engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    }
    if (engine.rootObjects().isEmpty())
        return -1;

    QQuickWindow *rootObject = static_cast<QQuickWindow *>(engine.rootObjects().first());
    QQuickItem *videoItem = rootObject->findChild<QQuickItem *>("videoItem");

    printf("C++: Pointer to QQuickItem: %p\n", videoItem);
    set_qmlglsink_widget(videoItem);

    return app.exec();
}
