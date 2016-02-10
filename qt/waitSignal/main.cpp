#include <QApplication>
#include <QQmlApplicationEngine>
#include <QtQml>

#include "quickeventloop.h"

int main(int argc, char *argv[])
{
    qmlRegisterType<QuickEventLoop>("Playground", 1, 0, "EventLoop");

    QApplication app(argc, argv);

    QQmlApplicationEngine engine;
    engine.load("qml/main.qml");
    return app.exec();
}
