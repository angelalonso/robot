package com.de.fonseca.robot_api_controller

import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.TextView

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val debug = findViewById<TextView>(R.id.debugTv);
        debug.setText(getDebug());
        val button = findViewById<Button>(R.id.button_cfg)
        button.setOnClickListener{
            val intent = Intent(this, SettingsActivity::class.java)
            startActivity(intent)
        }
    }

    fun sendTest(view: View) {

    }
    fun getDebug(): String {
        val result = "test";
        return result;
    }
}

