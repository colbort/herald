import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "../../"

Column {
    id: wrapper
    width: parent.width
    spacing: CmnCfg.smallMargin
    Row {
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.defaultMargin

        width: parent.width
        height: buttonCol.height
        StandardLabel {
            id: themeLabel
            text: qsTr("Theme")
            color: "black"
            anchors.top: parent.top
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
        }

        Item {
            height: 10
            width: wrapper.width * 0.66 - themeLabel.width
        }

        Column {
            id: buttonCol
            anchors.top: parent.top
            spacing: CmnCfg.smallMargin

            ConfRadio {
                text: qsTr("Dark")
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
            }

            ConfRadio {
                text: qsTr("Light")
                checked: true
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
            }
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }
}
