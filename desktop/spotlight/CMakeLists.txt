qt_add_executable(jadro-spotlight main.cpp)
qt_add_qml_module(jadro-spotlight
    URI Jadro.Spotlight
    VERSION 1.0
    QML_FILES Main.qml
)
target_link_libraries(jadro-spotlight PRIVATE Qt6::Quick)
