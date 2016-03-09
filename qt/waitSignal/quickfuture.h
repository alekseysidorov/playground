#ifndef QUICKFUTURE_H
#define QUICKFUTURE_H
#include <QObject>
#include <QJSValue>
#include <QMetaMethod>

class QuickFuture : public QObject
{
    Q_OBJECT
public:
    template <typename Object, typename Function>
    explicit QuickFuture(Object *from, Function method);

    Q_INVOKABLE void then(const QJSValue &callback);
private:
    template <typename T>
    void fillValueList(QJSValueList &list, T arg);
    template <typename Head, typename ...Args>
    void fillValueList(QJSValueList &list, Head arg, Args ...tail);
    template <typename ...Args>
    QJSValueList valueListFromArgs(Args ... args);

    QJSValue m_callback;
};

template <typename Object, typename Func>
QuickFuture::QuickFuture(Object *from, Func method)
{
    connect(from, method, [this](auto ...args) {
        auto values = valueListFromArgs(args...);
        if (m_callback.isCallable()) {
            m_callback.call(values);
        }
        deleteLater();
    });
}

template <typename T>
void QuickFuture::fillValueList(QJSValueList &list, T arg)
{
    list.push_back(arg);
}

template <typename Head, typename ...Args>
void QuickFuture::fillValueList(QJSValueList &list, Head arg, Args ...tail)
{
    list.push_back(arg);
    list.push_back(tail...);
}

template <typename ...Args>
QJSValueList QuickFuture::valueListFromArgs(Args ... args)
{
    QJSValueList list;
    fillValueList(list, args...);
    return list;
}

#endif // QUICKFUTURE_H
