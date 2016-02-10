#include "quickeventloop.h"

QuickEventLoop::QuickEventLoop(QObject *parent) : QEventLoop(parent)
{

}

int QuickEventLoop::exec(QEventLoop::ProcessEventsFlags flags)
{
    return QEventLoop::exec(flags);
}

bool QuickEventLoop::isRunning() const
{
    return QEventLoop::isRunning();
}
