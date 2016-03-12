#pragma once
#include <QAbstractListModel>
#include "blinker.h"
#include <array>
#include <QTimer>

class LedModel : public QAbstractListModel
{
    Q_OBJECT
public:
    enum class Role {
        Red = Qt::UserRole + 1,
        Green,
        Blue
    };

    LedModel(QObject *parent = nullptr);

    int rowCount(const QModelIndex &parent) const override;
    QVariant data(const QModelIndex &index, int role) const override;
    QHash<int, QByteArray> roleNames() const override;
private:
    void handleTimer();

    std::array<led_type, led_count> m_leds;
    blinker_config m_blinkerConfig;
    QTimer m_timer;
};
