package com.de.fonseca.robot_api_controller

import android.app.Application
import android.content.Context
import android.text.Editable
import android.util.Log
import android.widget.Toast
import khttp.responses.Response

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


    fun doCall(context: Context) {
        Thread({
            val response : Response = khttp.get("http://fconfig.co", data = "")
            val message : String = response.text
            Toast.makeText(context,message, Toast.LENGTH_SHORT).show()
            Log.d("TEST", message);
        }).start()


    }


}