import QtQuick 2.0

Item {
    id: root

    anchors.fill: parent

    property int rx
    property int ry
    property int distance: 0
    property int size: 16
    property int sizeVariation: 60

    property alias r: rLed.intensivity
    property alias g: gLed.intensivity
    property alias b: bLed.intensivity

    Led {
        id: rLed
        color: "red"

        rx: root.rx - root.distance
        ry: root.ry
        size: root.size
        sizeVariation: root.sizeVariation
    }

    Led {
        id: gLed
        color: "green"

        rx: root.rx + root.distance
        ry: root.ry
        size: root.size
        sizeVariation: root.sizeVariation
    }

    Led {
        id: bLed
        color: "blue"

        rx: root.rx
        ry: root.ry + root.distance
        size: root.size
        sizeVariation: root.sizeVariation
    }
}
