import QtQuick 2.0
import QtQuick.Particles 2.0
import QtQuick.Window 2.0

Window {
    height: 1000
    width: 800
    visible: true

    Rectangle {
        id: root
        anchors.fill: parent

        gradient: Gradient {
            GradientStop { position: 0; color: "#000020" }
            GradientStop { position: 1; color: "#000000" }
        }

        Repeater {
            model: 30

            RgbLed {
                rx: root.width / 2
                ry: 30 + (root.height - 30) / 30 * index

                r: (index % 3) ? 255 : 0
                g: (index % 5) ? 255 : 0
                b: (index % 2) ? 255 : 0
            }
        }

    }
}
