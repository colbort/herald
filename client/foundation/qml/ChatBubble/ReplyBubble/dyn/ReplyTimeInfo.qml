import QtQuick 2.14
import QtQuick.Layouts 1.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "../../../js/utils.mjs" as Utils

/// NOTE: Here be dragons, this depends on dynamic scoping
Row {
    spacing: 2
    Layout.bottomMargin: CmnCfg.smallPadding
    Layout.leftMargin: CmnCfg.smallMargin
    Layout.rightMargin: CmnCfg.smallMargin

    Label {
        id: replyTs
        Layout.margins: CmnCfg.smallMargin
        Layout.topMargin: 0
        font.pixelSize: 10
        text: modelData.replyType === 2 ? Utils.friendlyTimestamp(
                                              modelData.opInsertionTime) : ""
        color: CmnCfg.palette.darkGrey
    }

    Button {
        id: clock
        icon.source: modelData.opExpirationTime !== undefined ? "qrc:/countdown-icon-temp.svg" : ""
        icon.height: 16
        icon.width: 16
        icon.color: "grey"
        padding: 0
        background: Item {}
        anchors.verticalCenter: replyTs.verticalCenter
    }
}
