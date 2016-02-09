import QtQuick 2.3
import QtQuick.Window 2.2
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.1
import QtMultimedia 5.0

FocusScope {
    id: dialogWrapper

    default property alias screenData: container.data
    property alias title: titleLabel.text

    function show() {
        visible = true
    }

    width: Math.max(200, parent.width * 0.4)
    height: Math.min(container.height + headerBar.height + 36 + buttonsLayout.height, parent.height * 0.8)

    Rectangle {
        id: background

        border.color: "#F3F3F3"
        anchors.fill: parent
        clip: true

        Rectangle {
            id: headerBar

            color: "#F3F4F4"

            anchors {
                top: parent.top
                left: parent.left
                right: parent.right
            }
            height: titleLabel.implicitHeight + 12

            Label {
                id: titleLabel

                anchors {
                    top: parent.top
                    left: parent.left
                    right: parent.right
                    margins: 6
                }

                text: "It is my title"
                wrapMode: "WordWrap"
            }
        }

        ScrollView {
            id: scrollView

            frameVisible: false
            clip: true
            anchors {
                top: headerBar.bottom
                left: parent.left
                right: parent.right
                bottom: buttonsLayout.top
                margins: 12
            }

            Item {
                id: container

                width: headerBar.width - 60
                height: childrenRect.height

                onChildrenChanged: {
                    for (var i in children) {
                        var child = children[i]

                        child.anchors.left = container.left
                        child.anchors.right = container.right
                    }
                }
            }
        }

        ColumnLayout {
            id: buttonsLayout

            anchors {
                left: parent.left
                right: parent.right
                bottom: parent.bottom
                margins: 12
            }

            Button {
                text: "ok"
                Layout.fillWidth: true
            }
            Button {
                text: "cancel"
                Layout.fillWidth: true
            }
        }
    }
}

