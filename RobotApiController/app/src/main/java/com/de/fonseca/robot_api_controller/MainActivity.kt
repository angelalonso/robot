package com.de.fonseca.robot_api_controller

import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.github.kittinunf.fuel.Fuel


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
            Fuel.get("https://ifconfig.co")
                    .response { request, response, result ->
                        println(request)
                        println(response)
                        val (bytes, error) = result
                        if (bytes != null) {
                            println("[response bytes] ${String(bytes)}")
                            Toast.makeText(this,"${String(bytes)}", Toast.LENGTH_SHORT).show()
                        }
                    }
        }


    }

    fun sendTest(view: View) {

    }
    fun getDebug(): String {
        val result = Globals().getURL();
        return result;
    }

}

