import QtQuick 2.0
import QtQuick.Particles 2.0

Item {
    id: root

    property alias color: imageParticle.color
    property int intensivity: 255

    property int rx
    property int ry

    anchors.fill: parent
    opacity: intensivity / 255

    ParticleSystem { id: particleSystem }

    ImageParticle {
        id: imageParticle

        color: "cyan"
        system: particleSystem
        alpha: 0

        colorVariation: 0.3
        source: "qrc:///particleresources/star.png"
    }

    Emitter {
        id: trailsStars
        system: particleSystem

        emitRate: 150
        lifeSpan: 200


        x: root.rx
        y: root.ry

        velocity: PointDirection {xVariation: 2; yVariation: 2;}
        acceleration: PointDirection {xVariation: 5; yVariation: 5;}
        velocityFromMovement: 1

        size: 32
        sizeVariation: 4
    }
}
