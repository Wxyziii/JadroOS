#include <QGuiApplication>
#include <QQmlApplicationEngine>

int main(int argc, char **argv)
{
    QGuiApplication app(argc, argv);
    app.setApplicationName("Jadro Spotlight Prototype");
    QQmlApplicationEngine engine;
    engine.loadFromModule("Jadro.Spotlight", "Main");
    if (engine.rootObjects().isEmpty())
        return 1;
    return app.exec();
}
