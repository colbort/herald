/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class Contacts;
class Messages;

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
    Q_INVOKABLE QString contact_id(int row) const;
    Q_INVOKABLE QString name(int row) const;
    Q_INVOKABLE bool setName(int row, const QString& value);
    Q_INVOKABLE QByteArray profile_picture(int row) const;
    Q_INVOKABLE bool setProfile_picture(int row, const QByteArray& value);

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
    Q_PROPERTY(QString conversation_id READ conversation_id WRITE setConversation_id NOTIFY conversation_idChanged FINAL)
    explicit Messages(bool owned, QObject *parent);
public:
    explicit Messages(QObject *parent = nullptr);
    ~Messages();
    QString conversation_id() const;
    void setConversation_id(const QString& v);
    Q_INVOKABLE bool delete_message(quint64 row_index);
    Q_INVOKABLE bool send_message(const QString& recipient, const QString& body);

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
    Q_INVOKABLE qint64 message_id(int row) const;
    Q_INVOKABLE QString recipient(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void conversation_idChanged();
};
#endif // BINDINGS_H
