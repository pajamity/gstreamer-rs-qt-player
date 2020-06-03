#include "Bindings.h"

#include <QtCore/QFile>
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>
#include <QtQuick/QQuickItem>
#include <QtQuick/QQuickWindow>
#include <QtQml/qqml.h>

#include <iostream>
#include <glib-object.h>
// #include <gst/gst.h>

// exported functions
extern "C" {
    int main_cpp(const char* app, void* sink); // the function C++ source exports (and Rust source calls it)
}

int main_cpp(const char* appPath, void* sink)
{
    int argc = 1;
    char* argv[1] = { (char*)appPath };
    QGuiApplication app(argc, argv);
    qmlRegisterType<Player>("RustCode", 1, 0, "Player");

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

    std::cout << "Address of sink C++ was given by Rust: " << sink << std::endl;
    g_object_set(sink, "widget", videoItem, NULL);
    // set_property?

    return app.exec();
}