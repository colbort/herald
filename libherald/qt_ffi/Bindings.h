/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class Config;
class Contacts;
class Messages;
class NetworkHandle;

class Config : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(quint32 color READ color WRITE setColor NOTIFY colorChanged FINAL)
    Q_PROPERTY(quint32 colorscheme READ colorscheme WRITE setColorscheme NOTIFY colorschemeChanged FINAL)
    Q_PROPERTY(QString config_id READ config_id WRITE setConfig_id NOTIFY config_idChanged FINAL)
    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged FINAL)
    Q_PROPERTY(QString profile_picture READ profile_picture WRITE setProfile_picture NOTIFY profile_pictureChanged FINAL)
    explicit Config(bool owned, QObject *parent);
public:
    explicit Config(QObject *parent = nullptr);
    ~Config();
    quint32 color() const;
    void setColor(quint32 v);
    quint32 colorscheme() const;
    void setColorscheme(quint32 v);
    QString config_id() const;
    void setConfig_id(const QString& v);
    QString name() const;
    void setName(const QString& v);
    QString profile_picture() const;
    void setProfile_picture(const QString& v);
    Q_INVOKABLE bool exists() const;
Q_SIGNALS:
    void colorChanged();
    void colorschemeChanged();
    void config_idChanged();
    void nameChanged();
    void profile_pictureChanged();
};

class Contacts : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit Contacts(bool owned, QObject *parent);
public:
    explicit Contacts(QObject *parent = nullptr);
    ~Contacts();
    Q_INVOKABLE bool add(const QString& id);
    Q_INVOKABLE void clear_filter();
    Q_INVOKABLE bool filter(const QString& pattern, bool regex);
    Q_INVOKABLE bool remove(quint64 row_index);
    Q_INVOKABLE void remove_all();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool archive_status(int row) const;
    Q_INVOKABLE bool setArchive_status(int row, bool value);
    Q_INVOKABLE quint32 color(int row) const;
    Q_INVOKABLE bool setColor(int row, quint32 value);
    Q_INVOKABLE QString contact_id(int row) const;
    Q_INVOKABLE bool matched(int row) const;
    Q_INVOKABLE bool setMatched(int row, bool value);
    Q_INVOKABLE QString name(int row) const;
    Q_INVOKABLE bool setName(int row, const QString& value);
    Q_INVOKABLE QString profile_picture(int row) const;
    Q_INVOKABLE bool setProfile_picture(int row, const QString& value);

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};

class Messages : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QString conversationId READ conversationId WRITE setConversationId NOTIFY conversationIdChanged FINAL)
    explicit Messages(bool owned, QObject *parent);
public:
    explicit Messages(QObject *parent = nullptr);
    ~Messages();
    QString conversationId() const;
    void setConversationId(const QString& v);
    Q_INVOKABLE void clear_conversation_view();
    Q_INVOKABLE bool delete_conversation();
    Q_INVOKABLE bool delete_conversation_by_id(const QString& conversation_id);
    Q_INVOKABLE bool delete_message(quint64 row_index);
    Q_INVOKABLE bool insert_message(const QString& body);
    Q_INVOKABLE bool reply(const QString& body, qint64 op);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE QString author(int row) const;
    Q_INVOKABLE QString body(int row) const;
    Q_INVOKABLE qint64 epoch_timestamp_ms(int row) const;
    Q_INVOKABLE bool error_sending(int row) const;
    Q_INVOKABLE qint64 message_id(int row) const;
    Q_INVOKABLE QVariant op(int row) const;
    Q_INVOKABLE bool reached_recipient(int row) const;
    Q_INVOKABLE bool reached_server(int row) const;
    Q_INVOKABLE QString recipient(int row) const;
    Q_INVOKABLE qint64 uuid(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void conversationIdChanged();
};

class NetworkHandle : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(bool connectionPending READ connectionPending NOTIFY connectionPendingChanged FINAL)
    Q_PROPERTY(bool connectionUp READ connectionUp NOTIFY connectionUpChanged FINAL)
    Q_PROPERTY(bool newMessage READ newMessage NOTIFY newMessageChanged FINAL)
    explicit NetworkHandle(bool owned, QObject *parent);
public:
    explicit NetworkHandle(QObject *parent = nullptr);
    ~NetworkHandle();
    bool connectionPending() const;
    bool connectionUp() const;
    bool newMessage() const;
    Q_INVOKABLE bool send_message(const QString& message_body, const QString& to) const;
Q_SIGNALS:
    void connectionPendingChanged();
    void connectionUpChanged();
    void newMessageChanged();
};
#endif // BINDINGS_H
