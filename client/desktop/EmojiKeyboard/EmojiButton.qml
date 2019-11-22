import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

//PAUL: demagic all numbers and colors
Button {
    property bool takesModifier: false
    property string baseEmoji: ""
    property string emoji: takesModifier ? baseEmoji + CmnCfg.skinSwatchList[CmnCfg.skinSwatchIndex] : baseEmoji
    property color lowlight: "light gray"

    onClicked: maskShape.send(emoji)

    height: selector.height
    width: selector.width

    background: Rectangle {
        id: bg
        radius: 5
        opacity: parent.pressed ? 1.0 : 0.0
        anchors.fill: parent
        color: lowlight
    }

    Text {
        id: selector
        opacity: 1.0
        anchors.centerIn: parent
        color: "white"
        font.pixelSize: 15
        text: emoji
    }
}
