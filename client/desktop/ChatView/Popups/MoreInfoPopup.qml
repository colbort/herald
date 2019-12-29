import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports"
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/ChatBubble" as CB

Popup {
    id: moreInfoPopup
    property var convoMembers: parent.convoMembers
    property var messageData: parent.messageData
    property var receiptData
    property var ownedMessages: parent.ownedMessages
    property var outbound: messageData.author === Herald.config.configId

    height: chatView.height
    width: chatView.width
    anchors.centerIn: parent
    onClosed: messageInfoLoader.active = false

    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }

    Component.onCompleted: {
        receiptData = JSON.parse(moreInfoPopup.messageData.userReceipts)
    }

    Flickable {
        width: chatView.width
        height: chatView.height
        anchors.centerIn: parent
        contentWidth: width
        contentHeight: wrapperCol.height
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        Column {
            id: wrapperCol
            width: parent.width - CmnCfg.smallMargin * 2
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: CmnCfg.smallMargin
            topPadding: CmnCfg.smallMargin
            bottomPadding: CmnCfg.smallMargin

            Label {
                id: header
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.left: senderHeader.left
                text: "Message info"
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
                width: parent.width
                font.pixelSize: CmnCfg.headerFontSize

                IconButton {
                    anchors.right: parent.right
                    anchors.rightMargin: CmnCfg.smallMargin
                    id: xIcon
                    source: "qrc:/x-icon.svg"
                    icon.height: 26
                    icon.width: 26
                    fill: CmnCfg.palette.black
                    z: parent.z + 1
                    onClicked: {
                        messageInfoLoader.active = false
                        moreInfoPopup.close()
                    }
                }
            }
            CB.DefaultBubble {
                id: bubbleInfo
                convContainer: parent
                defaultWidth: parent.width
                width: parent.width
                messageModelData: moreInfoPopup.messageData
            }
            Label {
                id: senderHeader
                anchors.left: bubbleInfo.left
                text: "From:"
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            Item {
                id: author
                anchors.left: senderHeader.left
                height: CmnCfg.convoHeight
                width: parent.width
                Common.PlatonicRectangle {

                    boxTitle: messageData.authorName
                    boxColor: messageData.authorColor
                    picture: Utils.safeStringOrDefault(
                                 messageData.authorProfilePicture, "")
                    color: CmnCfg.palette.white
                    labelComponent: Av.ConversationLabel {
                        contactName: messageData.authorName
                        lastBody: "@" + messageData.author
                        labelColor: CmnCfg.palette.black
                        secondaryLabelColor: CmnCfg.palette.darkGrey
                        labelFontSize: CmnCfg.entityLabelSize
                    }
                    MouseArea {
                        id: hoverHandler
                    }

                    states: []
                }
            }

            Label {
                id: timeInfo
                anchors.left: author.left
                text: outbound ? "Sent at: " + Utils.userTime(
                                     messageData.insertionTime) : "Sent at: " + Utils.userTime(
                                     messageData.serverTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            Label {
                id: receiveInfo
                anchors.left: author.left
                text: "Received at: " + Utils.userTime(
                          messageData.insertionTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
                visible: !outbound
            }

            Label {
                id: expireInfo
                anchors.left: timeInfo.left
                visible: messageData.expirationTime !== undefined
                text: messageData.expirationTime
                      !== undefined ? "Expires at: " + Utils.userTime(
                                          messageData.expirationTime) : ""
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            Label {
                id: recHeader
                anchors.left: timeInfo.left
                text: "To:"
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            ListView {
                height: contentHeight
                width: parent.width
                model: convoMembers
                interactive: false
                highlightFollowsCurrentItem: false
                currentIndex: -1
                delegate: Item {
                    height: visible ? CmnCfg.convoHeight : 0
                    width: 250
                    visible: memberData.userId !== messageData.author
                    property var memberData: model
                    Common.PlatonicRectangle {
                        boxTitle: memberData.name
                        boxColor: memberData.color
                        picture: Utils.safeStringOrDefault(memberData.picture,
                                                           "")
                        property MouseArea hoverHandler
                        color: CmnCfg.palette.white
                        labelComponent: Av.ConversationLabel {
                            contactName: memberData.name
                            lastBody: "@" + memberData.userId
                            labelColor: CmnCfg.palette.black
                            secondaryLabelColor: CmnCfg.palette.darkGrey
                            labelFontSize: CmnCfg.entityLabelSize
                        }

                        Button {
                            anchors.right: parent.right
                            id: receipt
                            icon.source: Utils.receiptCodeSwitch(
                                             receiptData[memberData.userId])
                            icon.height: 16
                            icon.width: 16
                            icon.color: CmnCfg.palette.iconMatte
                            padding: 0

                            anchors.verticalCenter: parent.verticalCenter
                            background: Item {}
                        }

                        states: []
                    }
                }
            }
            ToolButton {
                anchors.horizontalCenter: parent.horizontalCenter
                contentItem: Text {
                    text: qsTr("DELETE MESSAGE")
                    color: CmnCfg.palette.white
                    font.pixelSize: CmnCfg.headerFontSize
                    font.family: CmnCfg.chatFont.name
                }

                background: Rectangle {
                    color: CmnCfg.palette.alertColor
                }
                onClicked: {
                    moreInfoPopup.close()
                    messageInfoLoader.active = false
                    ownedMessages.deleteMessage(ownedMessages.indexById(
                                                    messageData.msgId))
                }
            }
        }
    }
}