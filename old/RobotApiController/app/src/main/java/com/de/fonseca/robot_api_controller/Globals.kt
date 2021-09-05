package com.de.fonseca.robot_api_controller

import android.app.Application
import android.content.Context
import android.text.Editable
import android.util.Patterns
import android.webkit.URLUtil
import android.widget.Toast
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.httpGet
import com.github.kittinunf.fuel.httpPost
import com.github.kittinunf.result.Result

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

    fun apiCall(): String {
        var resp = "empty";
        Fuel.get(RobotURL)
                .response { request, response, result ->
                    println(request)
                    println(response)
                    val (bytes, error) = result
                    if (bytes != null) {
                        println("[response bytes] ${String(bytes)}")
                        resp = bytes.toString()
                    }
                }
        return resp
    }

    fun urlCall(): String {
        var resp = "empty";
        //if (URLUtil.isValidUrl(RobotURL)) {
        if (Patterns.WEB_URL.matcher(RobotURL).matches()) {

            var httpAsync = RobotURL
                    .httpPost()
                    .responseString { request, response, result ->
                        when (result) {
                            is Result.Failure -> {
                                val ex = result.getException()
                                resp = ex.toString();
                                println("----------------------------------- ${resp}")
                            }
                            is Result.Success -> {
                                val data = result.get()
                                resp = data;
                                println("----------------------------------- ${resp}")
                            }
                        }
                    }
            httpAsync.join()
        } else {
            resp = "Invalid URL!"
        }
        return resp
    }

    fun apiPostCall(params: String): String {
        var resp = "empty";
        //if (URLUtil.isValidUrl(RobotURL)) {
        if (Patterns.WEB_URL.matcher(RobotURL).matches()) {
            val apiPostURL = RobotURL + params;
            var httpAsync = apiPostURL
                    .httpPost()
                    .responseString { request, response, result ->
                        when (result) {
                            is Result.Failure -> {
                                val ex = result.getException()
                                resp = ex.toString();
                                println("----------------------------------- ${resp}")
                            }
                            is Result.Success -> {
                                val data = result.get()
                                resp = data;
                                println("----------------------------------- ${resp}")
                            }
                        }
                    }
            httpAsync.join()
        } else {
            resp = "Invalid URL!"
        }
        return resp
    }
    fun move(move: String): String {
        var resp = "";
        if (move == "fwd") {
            resp = apiPostCall("/do/motor_l=100,motor_r=100")
        } else if (move == "bwd") {
            resp = apiPostCall("/do/motor_l=-100,motor_r=-100")
        } else if (move == "right") {
            resp = apiPostCall("/do/motor_l=100,motor_r=-100")
        } else if (move == "left") {
            resp = apiPostCall("/do/motor_l=-100,motor_r=100")
        } else if (move == "stop") {
            resp = apiPostCall("/do/motor_l=0,motor_r=0")
        } else {
            resp = apiPostCall("")
        }
        return resp
    }
}