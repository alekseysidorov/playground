#include <QApplication>
#include <QQmlApplicationEngine>
#include <QtQml>

#include "quickeventloop.h"
#include "quickfuture.h"

class FutureTest : public QObject
{
    Q_OBJECT
public:
    FutureTest(QObject *parent = 0) : QObject(parent)
    {

    }
public slots:
    QuickFuture *doSlowOperation(const QString &name)
    {
        m_currentName = name;
        QTimer::singleShot(2000, this, SLOT(timerFinished()));

        auto future = new QuickFuture(this, &FutureTest::operationFinished);
        return future;
    }
signals:
    void operationFinished(bool, QString);
private slots:
    void timerFinished()
    {
        emit operationFinished(true, m_currentName);
    }
private:
    QString m_currentName;
};

int main(int argc, char *argv[])
{
    FutureTest test;

    qmlRegisterType<QuickEventLoop>("Playground", 1, 0, "EventLoop");
    qmlRegisterUncreatableType<QuickFuture>("Playground", 1, 0, "QuickFuture", "Use from C++ api");

    QApplication app(argc, argv);

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("test", &test);
    engine.load("qml/main.qml");
    return app.exec();
}

#include "main.moc"
