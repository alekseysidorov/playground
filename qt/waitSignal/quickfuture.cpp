#include "quickfuture.h"

void QuickFuture::then(const QJSValue &callback)
{
    m_callback = callback;
}
