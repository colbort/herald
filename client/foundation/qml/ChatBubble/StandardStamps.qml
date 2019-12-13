import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    spacing: 2
    Label {
        id: timestamp
        font.pixelSize: 10
        text: friendlyTimestamp
        color: CmnCfg.palette.darkGrey
    }

    Button {
        id: clock
        icon.source: expirationTime !== undefined ? "qrc:/countdown-icon-temp.svg" : ""
        icon.height: 16
        icon.width: 16
        icon.color: "grey"
        padding: 0
        anchors.margins: 0
        background: Item {}
        anchors.verticalCenter: timestamp.verticalCenter
    }

    Button {
        id: receipt
        icon.source: receiptImage
        icon.height: 16
        icon.width: 16
        icon.color: CmnCfg.palette.iconMatte
        padding: 0
        anchors.margins: 0
        background: Item {}
    }
}
