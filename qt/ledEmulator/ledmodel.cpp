#include "ledmodel.h"

LedModel::LedModel(QObject *parent) : QAbstractListModel (parent)
{
    blinker_init(&m_blinkerConfig);

    m_timer.setSingleShot(false);
    m_timer.setInterval(m_blinkerConfig.refresh_interval);
    connect(&m_timer, &QTimer::timeout, this, &LedModel::handleTimer);
    m_timer.start();
}

int LedModel::rowCount(const QModelIndex &parent) const
{
    return m_leds.size();
}

QVariant LedModel::data(const QModelIndex &index, int role) const
{
    const led_type &led = m_leds[index.row()];
    const auto namedRole = Role(role);

    switch (namedRole) {
    case Role::Red: return led.r;
    case Role::Green: return led.g;
    case Role::Blue: return led.b;
    }
    return 0;
}

QHash<int, QByteArray> LedModel::roleNames() const
{
    return {
        { int(Role::Red), "red" },
        { int(Role::Green), "green" },
        { int(Role::Blue), "blue" },
    };
}

void LedModel::handleTimer()
{
    blinker_tick(m_blinkerConfig.blinker_context, m_leds.data());

    emit dataChanged(createIndex(0, 0), createIndex(m_leds.size() - 1, 0));
}
