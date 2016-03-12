import QtQuick 2.5
import QtQuick.Particles 2.0
import QtQuick.Window 2.0

import Blinker 1.0

Window {
    height: 1000
    width: 1000
    visible: true

    LedModel {
        id: ledModel
    }

    Rectangle {
        id: root

        property int cx: width / 2
        property int cy: height / 2
        readonly property real dx: Math.cos(Math.PI * angle / 180)
        readonly property real dy: Math.sin(Math.PI * angle / 180)

        property real angle: 0

        anchors.fill: parent

        gradient: Gradient {
            GradientStop { position: 0; color: "#000020" }
            GradientStop { position: 1; color: "#000000" }
        }

        RotationAnimation on angle {
            loops: Animation.Infinite
            from: 0
            to: 360
            duration: 4000
            //running: false
        }

        Repeater {
            model: ledModel

            RgbLed {
                id: led

                rx: root.cx + (300 + index * 10) * root.dx
                ry: root.cy + (300 + index * 10) * root.dy

                r: red
                g: green
                b: blue
                size: 40
                sizeVariation: 0
            }
        }
    }

    Shortcut {
        sequence: StandardKey.Quit
        context: Qt.ApplicationShortcut
        onActivated: Qt.quit()
    }
}
