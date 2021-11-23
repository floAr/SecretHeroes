using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public enum SlotStatus
{
    EMPTY,
    CHARGED,
    OPENING,
    OPEN
}
public class DrawManager : MonoBehaviour
{
    public Camera DrawCamera;
    public Slot[] Slots;
    private Stack<Token> _tokensToDrop = new Stack<Token>();
    public float TimeOut = 0;
    public TMPro.TMP_Text PillCount;

    public void PrepareSlots(Token[] tokens)
    {
        //Slot1.SkillWeapons = tokens[0].weapons;
        //Slot1.SkillEngineering = tokens[0].engineering;
        //Slot1.SkillBiotech = tokens[0].biotech;
        //Slot1.SkillPsychics = tokens[0].psychics;
        //Slot1.DoUpdate();


        //Slot2.SkillWeapons = tokens[1].weapons;
        //Slot2.SkillEngineering = tokens[1].engineering;
        //Slot2.SkillBiotech = tokens[1].biotech;
        //Slot2.SkillPsychics = tokens[1].psychics;
        //Slot2.DoUpdate();


        //Slot3.SkillWeapons = tokens[2].weapons;
        //Slot3.SkillEngineering = tokens[2].engineering;
        //Slot3.SkillBiotech = tokens[2].biotech;
        //Slot3.SkillPsychics = tokens[2].psychics;
        //Slot3.DoUpdate();
        for (int i = 0; i < tokens.Length; i++)
        {
            _tokensToDrop.Push(tokens[i]);
        }
    }


    private string paddedNumbers(int number)
    {
        if (number < 10)
            return "00" + number.ToString();

        if (number < 100)
            return "0" + number.ToString();
        return number.ToString();
    }

    private void Update()
    {
        PillCount.text = paddedNumbers(_tokensToDrop.Count);
        bool allFilled = true;
        foreach (var item in Slots)
        {
            // if (item.Status == SlotStatus.EMPTY)
            // {
            //     allFilled = false;
            //     break;
            // }
        }

        if (!allFilled && _tokensToDrop.Count > 0)
        {
            TimeOut = Mathf.Max(0, TimeOut - Time.deltaTime);
            if (TimeOut <= 0)
            {
                var slot = Random.Range(0, 3);
                if (Slots[slot].Status == SlotStatus.EMPTY)
                {
                    Slots[slot].Fill(_tokensToDrop.Pop());
                    TimeOut = Random.Range(1, 2) + Random.value;
                }

            }
        }

    }
}
