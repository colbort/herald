import QtQuick.Controls 2.13
import Qt.labs.platform 1.1
import QtQuick 2.13
import LibHerald 1.0

// TODO: Make this a menu, perhaps.
// TODO: take a listModel, dumbo.
Rectangle {
    id: self
    property var model: ["no", "model", "defined"]
    property string currentItem: model[0]
    property var onSelected: function () {}
    property font labelFont: CmnCfg.defaultFont

    width: 100
    height: 40

    Menu {
        id: nativeMenu
    }

    // TODO: use instantiator
    Repeater {
        model: self.model
        Item {
            MenuItem {
                id: menuItem
                text: self.model[index]
                onTriggered: {
                    self.currentItem = self.model[index]
                    onSelected()
                }
                Component.onCompleted: {
                    nativeMenu.addItem(menuItem)
                }
            }
        }
    }

    MouseArea {
        anchors.fill: parent
        cursorShape: Qt.PointingHandCursor
        onClicked: nativeMenu.open()
    }

    Row {
        anchors.fill: parent
        spacing: CmnCfg.units.dp(2)

        Label {
            text: currentItem
            anchors.verticalCenter: parent.verticalCenter
            font: self.labelFont
            //font.family: CmnCfg.labelFont.name
            //font.pixelSize: CmnCfg.headerFontSize
        }

        Image {
            source: "qrc:/dropdown-arrow-icon.svg"
            width: 20
            height: 20
            anchors.verticalCenter: parent.verticalCenter
        }
    }
}
