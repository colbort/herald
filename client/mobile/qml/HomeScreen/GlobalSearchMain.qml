import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import QtQuick 2.12
import LibHerald 1.0
import "../ChatView" as ChatView
import "../Common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/Entity" as Entity

Page {
    id: searchView
    readonly property Component headerComponent: GlobalSearchHeader {
        parentPage: searchView
    }

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    Component.onCompleted: appRoot.router.searchView = searchView
    signal messageClicked(var searchConversationId, var searchMsgId)
    signal convoClicked(var searchConversationId)

    Flickable {
        anchors.fill: parent
        interactive: true
        contentHeight: contentCol.height
        boundsBehavior: Flickable.StopAtBounds

        ScrollBar.vertical: ScrollBar {
            policy: ScrollBar.AsNeeded
            width: CmnCfg.microMargin
        }

        Column {
            id: contentCol
            width: parent.width

            Text {
                text: qsTr("Conversations")
                anchors {
                    left: parent.left
                    leftMargin: CmnCfg.smallMargin
                    topMargin: CmnCfg.smallMargin
                }

                font.family: CmnCfg.labelFont.name
                font.weight: Font.DemiBold
                font.pixelSize: CmnCfg.labelFontSize
                color: CmnCfg.palette.offBlack
                bottomPadding: 0
            }

            ListView {
                height: contentHeight
                width: parent.width
                // conversations and messages are in a single column,
                // this needs to be uninteractive so that they scroll together
                interactive: false

                model: Herald.conversations
                delegate: ConversationItem {
                    property var conversationData: model
                    convoContent: ContentMap.get(model.conversationId)
                    tapEnabled: false
                    visible: conversationData.matched
                    height: visible ? CmnCfg.convoHeight : 0
                    itemTitle: !isNTS ? convoContent.title : qsTr("Note to Self")
                    colorCode: !isNTS ? convoContent.conversationColor : UserMap.get(
                                            Herald.config.configId).userColor
                    imageSource: !isNTS ? Utils.safeStringOrDefault(
                                              convoContent.picture,
                                              "") : Utils.safeStringOrDefault(
                                              Herald.config.profilePicture, "")
                    isGroup: !convoContent.pairwise
                    lastMsgDigest: convoContent.lastMsgDigest
                    isEmpty: lastMsgDigest === ""
                    TapHandler {
                        // TODO if state is fromComposeButton we should probably
                        // pop this view off the stack, so ChatView back button
                        // goes to home screen
                        onTapped: convoClicked(conversationData.conversationId)
                    }
                }
            }

            Repeater {
                model: ListModel {
                    ListElement {
                        flow: "group"
                        iconSource: "qrc:/contacts-icon.svg"
                        label: qsTr("Create new group")
                    }

                    ListElement {
                        flow: "contact"
                        iconSource: "qrc:/add-contact-icon.svg"
                        label: qsTr("Message new contact")
                    }
                }

                Rectangle {
                    height: CmnCfg.avatarSize
                    width: parent.width

                    Button {
                        id: createGroupIcon
                        icon.source: model.iconSource
                        icon.height: CmnCfg.iconSize
                        icon.width: CmnCfg.iconSize
                        anchors {
                            left: parent.left
                            leftMargin: CmnCfg.microMargin + (CmnCfg.avatarSize - CmnCfg.iconSize) / 2
                            verticalCenter: parent.verticalCenter
                        }

                        padding: 0
                        background: Item {}
                    }

                    Label {
                        text: model.label
                        font.family: CmnCfg.chatFont.name
                        font.pixelSize: CmnCfg.units.dp(15)
                        font.weight: Font.Medium
                        anchors {
                            left: createGroupIcon.right
                            leftMargin: (CmnCfg.avatarSize - CmnCfg.iconSize) / 2 + CmnCfg.defaultMargin
                            verticalCenter: parent.verticalCenter
                        }
                    }

                    TapHandler {
                        onTapped: {
                            if (model.flow === "group")
                                mainView.push(newGroupViewMain)
                            if (model.flow === "contact")
                                mainView.push(newContactViewMain)
                        }
                    }
                }
            }

            Text {
                text: qsTr("Messages")
                anchors {
                    left: parent.left
                    leftMargin: CmnCfg.smallMargin
                    topMargin: CmnCfg.smallMargin
                }
                visible: searchView.state === "fromComposeButton" ? false : true

                bottomPadding: 0
                font.family: CmnCfg.labelFont.name
                font.weight: Font.DemiBold
                font.pixelSize: CmnCfg.labelFontSize
                color: CmnCfg.palette.offBlack
            }

            ListView {
                id: messageSearchView
                height: contentHeight
                width: parent.width
                // conversations and messages are in a single column,
                // this needs to be uninteractive so that they scroll together
                interactive: false
                visible: searchView.state === "fromComposeButton" ? false : true

                model: Herald.messageSearch
                delegate: Item {
                    height: CmnCfg.convoHeight
                    width: parent.width

                    Common.PlatonicRectangle {
                        id: messageRectangle
                        boxTitle: model.conversationTitle
                        boxColor: model.conversationColor
                        picture: Utils.safeStringOrDefault(
                                     model.conversationPicture, "")
                        isGroupPicture: !model.conversationPairwise
                        isMessageResult: true

                        labelComponent: Entity.MessageSearchLabel {
                            conversationTitle: model.conversationTitle
                            timestamp: Utils.friendlyTimestamp(model.time)
                            labelColor: CmnCfg.palette.black
                            secondaryLabelColor: CmnCfg.palette.offBlack

                            beforeMatch: model.beforeFirstMatch
                            match: model.firstMatch
                            afterMatch: model.afterFirstMatch
                        }

                        TapHandler {
                            onTapped: {
                                messageClicked(model.conversation, model.msgId)
                            }
                        }
                    }
                }
            }
        }
    }

    states: [
        State {
            name: "default"
        },
        State {
            name: "fromComposeButton"
        }
    ]
}
