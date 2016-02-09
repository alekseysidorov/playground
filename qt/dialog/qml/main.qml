import QtQuick 2.4
import QtQuick.Controls 1.4

ApplicationWindow {
    id: window

    width: 600
    height: 800

    Component.onCompleted: {
        window.show()

        dialog.show()
    }

    Rectangle {
        anchors.fill: parent
        color: "white"

        Dialog {
            id: dialog

            anchors.centerIn: parent
            title: "Test Dialog with many fields"

            Label {
                text: qsTr("Qt Creator provides a cross-platform, complete integrated development environment (IDE) for application developers to create applications for multiple desktop, embedded, and mobile device platforms, such as Android and iOS. It is available for Linux, OS X and Windows operating systems. For more information, see Supported Platforms.

This manual also describes features that are only available if you have the appropriate Qt license. For more information, see Qt Creator Commercial Features.")
                wrapMode: Text.WordWrap
            }
        }
    }
}
