import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
import "../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils

Page {
    id: newGroupView
    height: mainView.height
    header: ToolBar {
        id: conversationViewHeader

        clip: true
        height: CmnCfg.toolbarHeight

        background: Rectangle {
            color: CmnCfg.palette.offBlack
        }

        RowLayout {
            anchors.fill: parent
            Row {
                Layout.alignment: Qt.AlignLeft
                Layout.leftMargin: CmnCfg.units.dp(12)
                spacing: CmnCfg.units.dp(16)
                IconButton {
                    id: backButton
                    color: CmnCfg.palette.iconFill
                    imageSource: "qrc:/back-arrow-icon.svg"
                    tapCallback: function () {
                        mainView.pop(null)
                    }
                }

                Label {
                    id: stateLabel
                    text: "New group"
                    font {
                        pointSize: CmnCfg.chatPreviewSize
                        family: CmnCfg.chatFont.name
                    }
                    anchors.verticalCenter: parent.verticalCenter
                    color: CmnCfg.palette.iconFill
                }
            }
        }
    }

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    GroupHeaderComponent {
        id: topRect
    }

    Rectangle {
        anchors.top: topRect.bottom
        id: bigDivider
        height: 1
        width: parent.width
        color: CmnCfg.palette.black
    }

    Column {

        topPadding: CmnCfg.units.dp(24)
        Component.onCompleted: herald.usersSearch.refresh()
        width: mainView.width - CmnCfg.units.dp(56)
        anchors.top: bigDivider.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        TextArea {
            id: groupSelectText
            leftPadding: 0
            placeholderText: "Add members"
            onTextChanged: {
                herald.usersSearch.filter = groupSelectText.text
                contactPopup.popup.open()
            }
        }

        Rectangle {
            height: 1
            width: parent.width
            color: "black"
        }

        ComboBox {
            id: contactPopup
            model: herald.usersSearch
            width: parent.width
            anchors.horizontalCenter: parent.horizontalCenter
            height: CmnCfg.units.dp(6)
            leftPadding: CmnCfg.margin

            //width: 200
            background: Rectangle {
                visible: false
            }

            indicator: Rectangle {
                height: 50
                width: 100
                visible: false
            }
            delegate: Rectangle {
                property var contactData: model
                height: visible ? CmnCfg.units.dp(48) : 0
                width: parent.width
                visible: matched
                // && contactData.userId !== herald.config.configId
                anchors {
                    // fill: parent
                    rightMargin: CmnCfg.units.dp(12)
                    leftMargin: CmnCfg.units.dp(12)
                }

                AvatarMain {
                    id: avatar
                    iconColor: CmnCfg.palette.avatarColors[contactData.color]
                    anchors.verticalCenter: parent.verticalCenter
                    initials: Utils.initialize(contactData.name)
                    size: CmnCfg.units.dp(36)
                    avatarHeight: CmnCfg.units.dp(36)

                    anchors {
                        right: parent.right
                        left: parent.left
                        leftMargin: CmnCfg.units.dp(12)
                    }

                    labelComponent: ConversationLabel {
                        contactName: contactData.name
                        labelColor: CmnCfg.palette.offBlack
                        labelSize: 14
                        lastBody: "@" + contactData.userId
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        //TODO: THIS WILL TAKE MODEL OWNED BY GLOBAL STATE
                        herald.conversationBuilder.addMember(contactData.userId)
                        contactPopup.popup.close()
                        herald.usersSearch.clearFilter()
                        groupSelectText.text = ""
                    }
                }
            }
        }
        //component for selected group members
        // FinalGroupList {}
    }
}
