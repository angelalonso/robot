package com.de.fonseca.robot_api_controller

import android.content.Intent
import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import androidx.appcompat.app.AppCompatActivity

class SettingsActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.settings_activity)

        val button = findViewById<Button>(R.id.button_back)
        val URLtext = findViewById<EditText>(R.id.et_URL);

        URLtext.setText(Globals().getURL());
        button.setOnClickListener{
            Globals().updateURL(URLtext.text);
            val intent = Intent(this, MainActivity::class.java)
            startActivity(intent)
        }

        supportActionBar?.setDisplayHomeAsUpEnabled(true)
    }

//

}