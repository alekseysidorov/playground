import QtQuick 2.5
import QtQuick.Controls 1.4

ParentWindow {
    id: window

    ListMerger {
        target: window
        property: "values"

        values: [
            "I am a parent"
        ]
    }
}
