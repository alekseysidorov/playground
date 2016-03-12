#include <QApplication>
#include <QQmlApplicationEngine>
#include <QtQml>

#include "ledmodel.h"

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);

    qmlRegisterType<LedModel>("Blinker", 1, 0, "LedModel");

    QQmlApplicationEngine engine;
    engine.load("qml/main.qml");
    return app.exec();
}
