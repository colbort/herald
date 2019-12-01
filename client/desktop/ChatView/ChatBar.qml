import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "qrc:/imports" as Imports
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils
import "Controls" as CVUtils

ToolBar {
    id: chatToolBar
    property var conversationItem
    property Messages ownedConversation: parent.ownedConversation

    height: CmnCfg.toolbarHeight
    z: CmnCfg.middleZ

    background: Rectangle {
        color: CmnCfg.palette.secondaryColor
    }

    RowLayout {
        id: buttonRow

        spacing: 12

        anchors {
            fill: parent
            leftMargin: CmnCfg.margin
            rightMargin: CmnCfg.smallMargin
        }

        AvatarMain {
            id: avatar
            size: 32
            avatarHeight: groupAvatar ? 30 : 32
            iconColor: CmnCfg.avatarColors[conversationItem.color]
            textColor: CmnCfg.palette.iconFill
            initials: conversationItem.title[0].toUpperCase()
            Layout.alignment: Qt.AlignLeft
            pfpPath: Utils.safeStringOrDefault(conversationItem.picture, "")
            groupAvatar: !conversationItem.pairwise
            anchors {
                margins: 16
            }
        }

        Label {
            id: uid
            font {
                bold: true
                family: CmnCfg.chatFont.name
                pixelSize: 18
            }
            Layout.alignment: Qt.AlignLeft
            Layout.fillWidth: true
            elide: Label.ElideRight
            text: conversationItem.title
            color: CmnCfg.palette.mainColor
        }

        Loader {
            id: searchLoader
            Layout.alignment: Qt.AlignLeft
            height: parent.height
        }

        Row {
            id: optionsRow
            spacing: CmnCfg.margin
            Layout.alignment: Qt.AlignRight
            height: parent.height

            Imports.ButtonForm {
                id: searchButton
                source: "qrc:/search-icon.svg"
                fill: CmnCfg.palette.paneColor
                topPadding: 1
                onClicked: chatToolBar.state = "searchState"
            }

            Imports.ButtonForm {
                id: timerButton
                source: timerMenu.chosenTimer
                fill: CmnCfg.palette.paneColor
                topPadding: 1
                onClicked: timerMenu.open()
            }

            Imports.TimerOptions {
                id: timerMenu
                conversationItem: chatToolBar.conversationItem
            }

            Imports.ButtonForm {
                id: convOptionsButton
                source: "qrc:/options-icon.svg"
                fill: CmnCfg.palette.paneColor
                onClicked: convOptionsMenu.open()
                Menu {
                    id: convOptionsMenu

                    MenuItem {
                        text: qsTr("Archive")
                    }

                    MenuItem {
                        text: qsTr("Clear History")
                        onTriggered: ownedConversation.clearConversationHistory(
                                         )
                    }
                }
            }
        }
    }

    states: State {
        name: "searchState"

        PropertyChanges {
            target: searchButton
            visible: false
        }

        PropertyChanges {
            target: searchLoader
            sourceComponent: chatSearchComponent
        }
    }
}
