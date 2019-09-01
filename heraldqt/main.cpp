#include "Bindings.h"
#include "qmlconstants.h"

#include <QtQml/qqml.h>
#include <QGuiApplication>
#include <QQmlApplicationEngine>

int main(int argc, char *argv[])
{
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QGuiApplication app(argc, argv);

    qmlRegisterType<Contacts>("LibHerald", 1, 0, "Contacts");
    qmlRegisterType<Messages>("LibHerald", 1, 0, "Messages");
    qmlRegisterType<Config>("LibHerald", 1, 0, "Config");
    qmlRegisterType<QmlConstants>("LibHerald", 1, 0, "AckTypes");
    qmlRegisterType<NetworkHandle>("LibHerald", 1, 0, "NetworkHandle");
    qmlRegisterSingletonType(QUrl("qrc:///common/CommonConfig.qml"), "LibHerald", 1, 0, "QmlCfg");

    app.setOrganizationName("Kalix Systems");
    app.setOrganizationDomain("kalix.io");
    app.setApplicationName("Herald");

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
