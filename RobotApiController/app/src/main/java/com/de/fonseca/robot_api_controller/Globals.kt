package com.de.fonseca.robot_api_controller

import android.app.Application
import android.text.Editable

class Globals : Application() {
    companion object {
        var RobotURL = "http://"
    }


    fun updateURL(newURL: Editable) {
        RobotURL = newURL.toString();
    }

    fun getURL(): String {
        return RobotURL;
    }
}