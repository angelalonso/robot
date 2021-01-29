package com.de.fonseca.robot_api_controller

import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity


class MainActivity : AppCompatActivity() {

    private val mContext: Context? = null

    fun getContext(): Context? {
        return mContext
    }

    companion object {
        var url_list: Array<String> = emptyArray();
    }
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val debug = findViewById<TextView>(R.id.debugTv);
        debug.setText(getDebug());
        val buttonCfg = findViewById<Button>(R.id.button_cfg)
        buttonCfg.setOnClickListener{
            val intent = Intent(this, SettingsActivity::class.java)
            startActivity(intent)
        }
        val buttonUp = findViewById<Button>(R.id.button_up)
        buttonUp.setOnClickListener{
            getContext()?.let { it1 -> Globals().doCall(it1) }
        }


    }

    fun sendTest(view: View) {

    }
    fun getDebug(): String {
        val result = Globals().getURL();
        return result;
    }

}

