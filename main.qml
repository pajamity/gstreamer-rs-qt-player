import QtQuick 2.6
import QtQuick.Window 2.2

import org.freedesktop.gstreamer.GLVideoItem 1.0

Window {
    visible: true
    width: 640
    height: 480
    title: qsTr("Player")

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

    Item {
        id: playpause
        objectName: "playpause"
        anchors.centerIn: parent
        width: 30
        height: 30
        text: "Play"

        property bool playing: false
        signal playStateChanged()

        function play() {
            playpause.playing = true
            playpause.text = "Pause"
        }

        function pause() {
            playpause.playing = false
            playpause.text = "Play"
        }

        onClicked: {
            if (playing) [
                pause()
            ] else {
                play()
            }
        }
    }
}
