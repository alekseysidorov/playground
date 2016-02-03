## List merge util for qml

#### Example
![Screenshot](/qt/listmerger/images/example.png)
#### Sample code
```qml
TestWindow {
    id: window

    visible: true
    width: 640
    height: 480
    title: qsTr("List merger demo")

    menuBar: MenuBar {
        Menu {
            title: qsTr("File")
            MenuItem {
                text: qsTr("&Open")
                onTriggered: console.log("Open action triggered");
            }
            MenuItem {
                text: qsTr("Exit")
                onTriggered: Qt.quit();
            }
        }
    }


    ListMerger {
        target: window
        property: "values"

        values: [
            "I am a child",
        ]
    }

    ListMerger {
        target: window
        property: "values"

        values: [
            "I am a grandchild",
        ]
    }

    Column {
        anchors.centerIn: parent
        spacing: 6

        Repeater {
            model: values

            Label { text: modelData }
        }
    }
}
```
