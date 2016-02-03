import QtQuick 2.0
import "storage.js" as Storage

QtObject {
    id: merger

    property QtObject target
    property string property

    property variant values

    property QtObject __oldTarget
    property string __oldProperty

    Component.onCompleted: {
        if (merger.target && merger.property) {
            Storage.updateTargetList(merger.target, merger.property)
        }
    }
    Component.onDestruction: {
        if (merger.target && merger.property) {
            Storage.removeTargetObj(merger.target, merger.property, merger)
        }
    }
    onValuesChanged: Storage.updateTargetList(merger.target, merger.property)
    onTargetChanged: {
        if (merger.property) {
            if (__oldTarget) {
                Storage.removeTargetObj(__oldTarget, merger.property, merger)
            }
            Storage.addTargetObj(merger.target, merger.property, merger)
        }
        __oldTarget = target
    }
    onPropertyChanged: {
        if (merger.target && merger.property) {
            if (__oldProperty) {
                Storage.removeTargetObj(target, __oldProperty, merger)
            }
            Storage.addTargetObj(merger.target, merger.property, merger)
        }
        __oldProperty = merger.property
    }
}
