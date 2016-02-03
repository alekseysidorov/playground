import QtQuick 2.5
import QtQuick.Controls 1.4

ApplicationWindow {
    id: window

    property var values: []

    ListMerger {
        target: window
        property: "values"

        values: [
            "I am a grandparent"
        ]
    }
}
