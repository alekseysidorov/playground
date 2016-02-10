#ifndef QUICKEVENTLOOP_H
#define QUICKEVENTLOOP_H

#include <QEventLoop>


class QuickEventLoop : public QEventLoop
{
    Q_OBJECT
    Q_ENUMS(QuickEventLoop::ProcessEventsFlag)
public:
    QuickEventLoop(QObject *parent = 0);

public slots:
    int exec(ProcessEventsFlags flags = AllEvents);
    bool isRunning() const;
};

#endif // QUICKEVENTLOOP_H
