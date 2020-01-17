import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as JS

// TODO:
// there are some loose magic numbers
// hanging around in the font sizes. move those to CmnCfg
// TODO:
// move the property translation functions into
// some common js directory , receipt urls are not numbers, nor are timestamps

// TODO this should probably be called something to reflect that it's also used
// for contacts, not just conversations
Item {
    id: wrapper
    // the group name or displayName of the conversation
    property string convoTitle

    property color labelColor: CmnCfg.palette.black
    property color minorTextColor: CmnCfg.palette.offBlack
    property int labelFontSize: CmnCfg.entityLabelSize
    property int subLabelFontSize: CmnCfg.entitySubLabelSize
    property alias receiptFill: receiptImage.icon.color

    property bool typeActive: false
    // json summary
    property string lastMsgDigest

    // This component expects one of the following groups of properties,
    // either a lastMsgDigest property (a JSON object), or the subsequent group of
    // properties calculated from it.

    // OPTION 1: lastMsgDigest
    // the bundle this label represents.
    property var lastMessage: lastMsgDigest !== "" ? JSON.parse(
                                                         lastMsgDigest) : undefined
    property bool isEmpty: true

    readonly property bool __init: !isEmpty && lastMessage !== undefined

    // OPTION 2: lastReceipt, outbound, lastAuthor, lastTimestamp, and lastBody
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: !__init ? -1 : lastMessage.status
    // true if the last message was sent by the logged-in user
    property bool outbound: !__init ? false : lastMessage.author === Herald.config.configId
    // user who sent the last message in the conversation
    property string lastAuthor: {
        if (!__init) {
            return ""
        }

        if (outbound)
            return qsTr('You')

        if (!isEmpty)
            return Herald.users.nameById(lastMessage.author)

        return ''
    }
    // the previous latest human readable timestamp, or the empty string
    property string lastTimestamp: __init ? JS.friendlyTimestamp(
                                                lastMessage.time) : ""

    // the previous message of the conversation, or the empty string
    property string lastBody: {
        if (!__init)
            return ""

        if (lastMessage.auxCode !== null) {
            return "<i>" + lastAuthor + JS.auxStringShort(
                        lastMessage.auxCode) + "</i>"
        }

        if (lastMessage.body === null && lastMessage.hasAttachments) {
            return lastAuthor + ": " + "<i>Media message</i>"
        }

        if (lastMessage.body !== null) {
            return lastAuthor + ": " + lastMessage.body
        }

        return ''
    }

    GridLayout {
        id: nameGrid
        anchors.top: parent.top
        anchors.left: parent.left
        Label {
            id: name
            font {
                family: CmnCfg.chatFont.name
                pixelSize: labelFontSize
                weight: Font.Medium
            }
            Layout.maximumWidth: wrapper.width - ts.width
            elide: "ElideRight"
            text: convoTitle
            color: labelColor
            padding: 0
        }
    }
    Label {
        anchors.right: parent.right
        anchors.top: nameGrid.top
        id: ts
        font {
            family: CmnCfg.chatFont.name
            pixelSize: CmnCfg.minorTextSize
        }
        text: lastTimestamp
        padding: 0
        color: minorTextColor
    }

    Loader {
        id: textLoader
        active: !typeActive
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        width: active ? item.implicitWidth : 0
        height: active ? item.implicitHeight : 0

        sourceComponent: GridLayout {
            id: bodyGrid
            Label {

                id: bodyText
                font {
                    family: CmnCfg.chatFont.name
                    pixelSize: subLabelFontSize
                }
                background: Item {}
                elide: "ElideRight"
                text: lastBody
                Layout.maximumWidth: wrapper.width - CmnCfg.defaultMargin * 2
                color: labelColor
                textFormat: Text.StyledText
                padding: 0
            }
        }
    }

    Loader {
        id: typeLoader
        active: typeActive
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        width: active ? item.implicitWidth : 0
        height: active ? item.implicitHeight : 0
        sourceComponent: GridLayout {
            id: typeGrid
            Label {
                id: type
                font {
                    family: CmnCfg.chatFont.name
                    pixelSize: subLabelFontSize
                }
                background: Item {}
                elide: "ElideRight"
                text: "<i>" + qsTr("Someone is typing") + "...</i>"
                Layout.maximumWidth: wrapper.width - CmnCfg.defaultMargin * 2
                color: labelColor
                textFormat: Text.StyledText
                padding: 0
            }
        }
    }

    Button {
        id: receiptImage
        visible: outbound && !typeActive
        icon.source: JS.receiptCodeSwitch(lastReceipt)
        icon.height: CmnCfg.units.dp(14)
        icon.width: CmnCfg.units.dp(14)
        anchors.bottom: textLoader.bottom
        anchors.right: parent.right
        icon.color: CmnCfg.palette.iconFill
        padding: 0
        background: Item {}
    }
} //}
