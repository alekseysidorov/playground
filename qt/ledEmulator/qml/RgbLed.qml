import QtQuick 2.0

Item {
    id: root

    anchors.fill: parent

    property int rx
    property int ry
    property int size: 0

    property alias r: rLed.intensivity
    property alias g: gLed.intensivity
    property alias b: bLed.intensivity

    Led {
        id: rLed
        color: "red"

        rx: root.rx - size
        ry: root.ry
    }

    Led {
        id: gLed
        color: "green"

        rx: root.rx + size
        ry: root.ry
    }

    Led {
        id: bLed
        color: "blue"

        rx: root.rx
        ry: root.ry + size
    }
}
