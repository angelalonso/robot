package com.de.fonseca.robot_api_controller

import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.view.MotionEvent
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
        //buttonUp.setOnClickListener{
        //    val apiReturn = Globals().move("fwd");
        //    Toast.makeText(this,apiReturn, Toast.LENGTH_SHORT).show()
        //}
        buttonUp.setOnTouchListener(object: View.OnTouchListener {
            override fun onTouch(v: View?, event: MotionEvent?): Boolean {
                when (event?.action) {
                    MotionEvent.ACTION_DOWN -> {
                        val apiReturn = Globals().move("fwd");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()
                    }
                    MotionEvent.ACTION_UP -> {
                        val apiReturn = Globals().move("stop");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()

                    }
                }

                return v?.onTouchEvent(event) ?: true
            }
        })
        val buttonDown = findViewById<Button>(R.id.button_down)
        buttonDown.setOnTouchListener(object: View.OnTouchListener {
            override fun onTouch(v: View?, event: MotionEvent?): Boolean {
                when (event?.action) {
                    MotionEvent.ACTION_DOWN -> {
                        val apiReturn = Globals().move("bwd");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()
                    }
                    MotionEvent.ACTION_UP -> {
                        val apiReturn = Globals().move("stop");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()

                    }
                }

                return v?.onTouchEvent(event) ?: true
            }
        })
        val buttonRight = findViewById<Button>(R.id.button_right)
        buttonRight.setOnTouchListener(object: View.OnTouchListener {
            override fun onTouch(v: View?, event: MotionEvent?): Boolean {
                when (event?.action) {
                    MotionEvent.ACTION_DOWN -> {
                        val apiReturn = Globals().move("right");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()
                    }
                    MotionEvent.ACTION_UP -> {
                        val apiReturn = Globals().move("stop");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()

                    }
                }

                return v?.onTouchEvent(event) ?: true
            }
        })
        val buttonLeft = findViewById<Button>(R.id.button_left)
        buttonLeft.setOnTouchListener(object: View.OnTouchListener {
            override fun onTouch(v: View?, event: MotionEvent?): Boolean {
                when (event?.action) {
                    MotionEvent.ACTION_DOWN -> {
                        val apiReturn = Globals().move("left");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()
                    }
                    MotionEvent.ACTION_UP -> {
                        val apiReturn = Globals().move("stop");
                        Toast.makeText(this@MainActivity,apiReturn, Toast.LENGTH_SHORT).show()

                    }
                }

                return v?.onTouchEvent(event) ?: true
            }
        })

    }

    fun getDebug(): String {
        val result = Globals().getURL();
        return result;
    }

}

