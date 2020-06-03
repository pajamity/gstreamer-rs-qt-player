import QtQuick 2.14
import QtQuick.Window 2.14
import QtQuick.Controls 2.14
import RustCode 1.0

import org.freedesktop.gstreamer.GLVideoItem 1.0

Window {
    visible: true
    width: 640
    height: 480
    title: qsTr("Player")

    Player {
        id: player
    }

    Item {
        anchors.fill: parent

        GstGLVideoItem {
            id: videoItem
            objectName: "videoItem"
            anchors.centerIn: parent
            width: parent.width
            height: parent.height
        }
    }

    Button {
        id: playpause
        objectName: "playpause"
        anchors.centerIn: parent
        width: 30
        height: 30
        text: "Play"

        property bool playing: false

        function play() {
            playpause.playing = true
            playpause.text = "Pause"
            player.play()
        }

        function pause() {
            playpause.playing = false
            playpause.text = "Play"
            player.pause()
        }
        
        onClicked: {
            if (playing) {
                pause()
            } else {
                play()
            }
        }
    }
}
