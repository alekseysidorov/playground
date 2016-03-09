import QtQuick 2.4
import QtQuick.Controls 1.4

import Playground 1.0

ApplicationWindow {
    id: window

    function waitFor(obj, sig) {
        rect.enabled = false
        loop.waitFor(obj, sig)
        rect.enabled = true
    }

    width: 600
    height: 800

    Component.onCompleted: {
        window.show()
    }

    EventLoop {
        id: loop

        function waitFor(obj, sig) {
            if (loop.isRunning()) {
                console.error("EventLoop is already running.")
                return
            }
            obj[sig].connect(loop.quit)
            loop.exec()
        }
    }

    Timer {
        id: timer

        repeat: false
        interval: 2000
    }

    Rectangle {
        id: rect

        anchors.fill: parent
        color: "white"

        Column {
            anchors.centerIn: parent

            Button {
                text: "Click me"

                onClicked: {
                    timer.start()
                    waitFor(timer, "triggered")
                    label.text += "Clicked\n"

                    test.doSlowOperation("I am slow!").then(function (set, string) {
                        console.log(set, string);
                    });
                }
            }

            Label {
                id: label
            }
        }
    }
}
