/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class Player;

class Player : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit Player(bool owned, QObject *parent);
public:
    explicit Player(QObject *parent = nullptr);
    ~Player();
    Q_INVOKABLE void pause();
    Q_INVOKABLE void play();
Q_SIGNALS:
};
#endif // BINDINGS_H
